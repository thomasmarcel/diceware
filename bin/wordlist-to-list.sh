#!/usr/bin/env bash

grep -e "^[0-6].*" $1 | while read line; do
	dices=$(echo $line | cut -d" " -f1)
	word=$(echo $line | cut -d" " -f2)
	echo "{WordDict {dices: String::from(\"$dices\"), word: String::from(\"$word\")} },"
done
