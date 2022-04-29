#!/bin/bash

for i in {0..10..1}
do
	/bin/mkdir -p test"$i"
done

echo "Hidden File" > .test-file.txt

j=10
for i in {0..10..1}
do
	cd test"$i"
	echo "Testing Unix shell for test0$i" > test-file0"$i"
	echo "Testing Unix shell for test1$i" > test-file1"$i"
	echo "Testing Unix shell for test2$i" > test-file2"$i"
	ln -s ../test"$j"/test-file1"$j" sym-test-1"$j"
	cd ..
	j=$((j-1))
done

j=1
for i in {0..10..1}
do
	i=$((j+1))

	if [ $i -gt 10 ]; then
		break
	fi
	echo "$i"
	cd test"$i"
	/bin/mkdir -p child-test"$i"
	cd child-test"$i"
	echo "Testing Unix shell for child-test$i" > child-test-file0"$i"
	ln -s ../test-file0"$i" sym-test-0"$j"
	cd ..
	cd ..
	j=$((j+"$i"))
done
