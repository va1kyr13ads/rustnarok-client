param(
    [Parameter(Mandatory = $true)][string]$Executable,
    [Parameter(Mandatory = $true)][string]$LogoBase64Path
)
$ErrorActionPreference = "Stop"
$Executable = (Resolve-Path -LiteralPath $Executable).Path
$LogoBase64Path = (Resolve-Path -LiteralPath $LogoBase64Path).Path

# The approved MYgene emblem is committed as a small PNG stored in base64
# so it can be reviewed and managed through GitHub's text-file interface.
$png = [Convert]::FromBase64String(([IO.File]::ReadAllText($LogoBase64Path)).Trim())
if ($png.Length -lt 32 -or [Text.Encoding]::ASCII.GetString($png, 1, 3) -ne "PNG") {
    throw "MYgene icon source is not a PNG"
}

# ICO supports PNG-encoded frames. A 64px PNG is a valid RT_ICON resource.
# GRPICONDIR differs from an ICO file directory only in its last WORD (icon ID).
$group = New-Object IO.MemoryStream
$writer = New-Object IO.BinaryWriter($group)
$writer.Write([UInt16]0)         # reserved
$writer.Write([UInt16]1)         # icon
$writer.Write([UInt16]1)         # images
$writer.Write([byte]64)          # width
$writer.Write([byte]64)          # height
$writer.Write([byte]0)           # palette
$writer.Write([byte]0)           # reserved
$writer.Write([UInt16]1)         # planes
$writer.Write([UInt16]32)        # bit depth
$writer.Write([UInt32]$png.Length)
$writer.Write([UInt16]1)         # resource icon ID
$writer.Flush()
$groupBytes = $group.ToArray()
$writer.Dispose()

Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
public static class MYgeneResource {
    [DllImport("kernel32.dll", CharSet=CharSet.Unicode, SetLastError=true)]
    public static extern IntPtr BeginUpdateResource(string filename, bool deleteExistingResources);
    [DllImport("kernel32.dll", SetLastError=true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    public static extern bool UpdateResource(IntPtr update, IntPtr type, IntPtr name,
        ushort language, byte[] data, uint size);
    [DllImport("kernel32.dll", SetLastError=true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    public static extern bool EndUpdateResource(IntPtr update, bool discard);
}
'@

$handle = [MYgeneResource]::BeginUpdateResource($Executable, $false)
if ($handle -eq [IntPtr]::Zero) {
    throw "BeginUpdateResource failed: $([Runtime.InteropServices.Marshal]::GetLastWin32Error())"
}
$commit = $false
try {
    if (-not [MYgeneResource]::UpdateResource($handle, [IntPtr]3, [IntPtr]1, [UInt16]0,
            $png, [UInt32]$png.Length)) {
        throw "RT_ICON failed: $([Runtime.InteropServices.Marshal]::GetLastWin32Error())"
    }
    if (-not [MYgeneResource]::UpdateResource($handle, [IntPtr]14, [IntPtr]1, [UInt16]0,
            $groupBytes, [UInt32]$groupBytes.Length)) {
        throw "RT_GROUP_ICON failed: $([Runtime.InteropServices.Marshal]::GetLastWin32Error())"
    }
    $commit = $true
}
finally {
    if (-not [MYgeneResource]::EndUpdateResource($handle, -not $commit)) {
        throw "EndUpdateResource failed: $([Runtime.InteropServices.Marshal]::GetLastWin32Error())"
    }
}
Write-Host "MYgene executable icon embedded: $Executable" -ForegroundColor Green
