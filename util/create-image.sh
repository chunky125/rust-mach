sudo losetup -P /dev/loop0 /home/chunky/Devel/qemu-images/raspi.img
sudo mount /dev/loop0p1 /mnt/loop
sudo cp -v target/aarch64-unknown-none/debug/rust-mach /mnt/loop
sudo umount /dev/loop0p1
sudo losetup -D
qemu-system-aarch64 -machine raspi3 -serial vc -serial vc -kernel /home/chunky/Devel/u-boot/u-boot.bin -sd /home/chunky/Devel/qemu-images/raspi.img -gdb tcp:127.0.0.1:1875
