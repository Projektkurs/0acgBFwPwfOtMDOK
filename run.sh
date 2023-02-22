# run.sh - automated starting script
#
# Copyright 2022 by Ben Mattes Krusekamp <ben.krause05@gmail.com>
#
# note that smartclock needs to be built manually with 

#if [ $1 == buildflutter ]; then 
#    export PATH=$PATH:~/dev/flutter/bin #if flutter is already in the PATH or in a different location, delete this line
#    bash -c "cd smartclock ;flutter build linux --dart-define=isepaper=true"
#fi

sudo echo "got root rights"
bash -c "cd z8CEK6uP25BmnnVk; ./build/linux/arm64/release/bundle/helperpaper" &
sleep 10
wmctrl -r helperpaper -b add,fullscreen
sleep 5
#cargo build --release -Z sparse-registry #sparse-registry reduces compile times massivly on a raspberry pi
#bash -c "cd webserver; cargo build --release -Z sparse-registry" 
echo "starting ePaper communication"
bash -c 'cd ePaper-com; cargo build --release -Z sparse-registry; sudo ./target/release/ePaper-com' &
echo "starting ePaper communication"
bash -c 'cd webserver; cargo run --release -Z sparse-registry' &
bash -c 'cd x11-imageprocessor; cargo run --release -Z sparse-registry' &
#allow root to start smartclock
#xhost local:root
#start execution
#sudo bash -c "export HOME=$HOME ;cd webserver ; target/release/webserver" &
#sudo su -c "xauth add \$(xauth -f ~pk/.Xauthority list | tail -1) ; target/release/rust ;" 
#sudo su -c "target/release/rust"
