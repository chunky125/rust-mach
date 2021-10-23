if /home/chunky/Devel/grub-prefix/bin/grub-file --is-x86-multiboot2 ./target/aarch64-unknown-none/debug/rust-mach; then
	echo multiboot confirmed
else
	echo not multiboot
fi
if /home/chunky/Devel/grub-prefix/bin/grub-file --is-x86-multiboot2 ../qemu-images/orig/gnumach; then
	echo original has multiboot
else
	echo original did not work
fi
