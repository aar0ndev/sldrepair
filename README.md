# Solidworks Repair Utility

A small program to make repairing SOLIDWORKS easier for end users.

Using an administrative image typically requires repairing from the source installer, finding the appropriate `.msi` in `C:\Windows\Installer`, or automating it through some script. This chooses the third option, but is not so specific that it requires to hard-code specific version information. The goal is to make repairs simple and painless and avoid the need for tedious instructions or screen share sessions.

# Running

End users should be given a copy of `sldrepair.exe` and then instructed to run (double-click) the file.

Users will first be prompted to select a product. The down arrow <kbd>&downarrow;</kbd> and up arrow <kbd>&uparrow;</kbd> keys are used to select the product to repair. Clicking <kbd>Enter</kbd> begins the process. 

The utility calls `msiexec /f` behind the scenes to start the actual repair. After the repair is finished the user can close the console window or press `Enter` to exit.

# Building

`cargo build --release`

