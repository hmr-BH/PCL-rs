Add-Type @"
using System;
using System.Runtime.InteropServices;
using System.Text;
public class Win32 {
  public delegate bool EnumWindowsProc(IntPtr hWnd, IntPtr lParam);
  [DllImport("user32.dll")] public static extern bool EnumWindows(EnumWindowsProc lpEnumFunc, IntPtr lParam);
  [DllImport("user32.dll")] public static extern int GetWindowText(IntPtr hWnd, StringBuilder lpString, int nMaxCount);
  [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr hWnd);
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr hWnd, out RECT lpRect);
  [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr hWnd);
  [DllImport("user32.dll")] public static extern bool ShowWindowAsync(IntPtr hWnd, int nCmdShow);
  [StructLayout(LayoutKind.Sequential)]
  public struct RECT { public int Left, Top, Right, Bottom; }
}
"@

$target = $null
$callback = [Win32+EnumWindowsProc] {
    param($hWnd, $lParam)
    if ([Win32]::IsWindowVisible($hWnd) -and $target -eq $null) {
        $sb = New-Object System.Text.StringBuilder(256)
        [Win32]::GetWindowText($hWnd, $sb, 256) | Out-Null
        $title = $sb.ToString()
        if ($title -like "*Slint*") { $script:target = $hWnd }
    }
    return $true
}
[Win32]::EnumWindows($callback, 0) | Out-Null

if ($target -eq $null) { exit 1 }
[Win32]::ShowWindowAsync($target, 9) | Out-Null
Start-Sleep -m 300
[Win32]::SetForegroundWindow($target) | Out-Null
Start-Sleep -m 300

$rect = New-Object Win32+RECT
[Win32]::GetWindowRect($target, [ref]$rect) | Out-Null

Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
$bounds = [System.Drawing.Rectangle]::FromLTRB($rect.Left, $rect.Top, $rect.Right, $rect.Bottom)
$bitmap = New-Object System.Drawing.Bitmap($bounds.Width, $bounds.Height)
$graphics = [System.Drawing.Graphics]::FromImage($bitmap)
$graphics.CopyFromScreen($bounds.Location, [System.Drawing.Point]::Empty, $bounds.Size)
$bitmap.Save("D:/Projects/PCL-rs/screenshot.png")
$graphics.Dispose()
$bitmap.Dispose()
Write-Host "screenshot saved"
