#! /usr/bin/env bash 

set -eu 

client_pid=$PPID
tmp_dir=$PWD/.tmp

case "$1" in 
    start) 
        mkdir -p $tmp_dir/pids 
        mkdir -p $tmp_dir/database
        touch $tmp_dir/pids/$client_pid

        if [ $(ls -l $tmp_dir/pids | grep ^- | wc -l) == "1" ]
        then
          echo 'starting database'
          pg_ctl -D $tmp_dir/database -l $tmp_dir/logfile -o "--unix_socket_directories='$tmp_dir'" start
        fi
        ;; 
    stop)
        rm $tmp_dir/pids/$client_pid

        if [ $(ls -l $tmp_dir/pids | grep ^- | wc -l) == "0" ]
        then
          echo 'stopping database'
          pg_ctl -D $tmp_dir/database stop
        fi
        ;; 
    *)
        echo "nuh uh"
        exit 1
        ;;
esac
