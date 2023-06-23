## KVM

## Backup

1. Check a vm name: `virsh list --all`
2. Stop the machine: `virsh shutdown my-vm`
3. Backup XML: `virsh dumpxml my-vm > /backup/my-vm.xml`
4. Check where disks files live: `virsh domblklist my-vm`
5. Copy the `qcow2` vm file: `sudo cp --sparse=always /var/lib/libvirt/images/my-vm.qcow2 /backup/`


## Restore

1. If vm exists - undefine it: `virsh undefine my-vm`
2. Copy the disk: `sudo cp --sparse=always /backup/my-vm.qcow2 /var/lib/libvirt/images/`
3. Create XML: `virsh define –file /backup/my-vm.xml`
