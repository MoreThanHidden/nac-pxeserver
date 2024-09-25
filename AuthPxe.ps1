param(
    $pin,
    $asset,
    $macAddress,
    $ipAddress,
    $serial,
    $manufacturer,
    $product
)

if ($pin -eq "1234") {
    # Add anything you want to do here such as a MAB Authentication Request to ISE


    # Return iPXE script to boot from SCCM
    return "#!ipxe`n`nchain http://`${next-server}:4433/SCCMBoot"
} else {
    return "#!ipxe`n`necho PIN $pin is invalid"
}