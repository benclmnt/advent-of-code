#! /usr/bin/env bash

f() {
  awk -v i=$1 -v bit=$2 -F '' '
  {
    if ($i == bit) {
      print $0;
    }
  }'
}


TMP=tmp
TMP2=tmp2

for j in {0..1}; do
  \cp $1 $TMP

  for i in {1..12}; do
    lines=$(cat $TMP | wc -l)
    if (( $lines == 1 )); then break; fi

    ones=$(cat $TMP | f $i 1 | wc -l)
    if (( $ones < $lines - $ones )); then
      cat $TMP | f $i $j >| $TMP2
    else
      cat $TMP | f $i $((1-$j)) >| $TMP2
    fi
    \cp $TMP2 $TMP
  done

  res=$(echo "ibase=2; $(cat $TMP)" | bc)
  if [ $j -eq 0 ]; then oxygen=$res; else co2=$res; fi
done

echo $(echo "$oxygen * $co2" | bc)
rm $TMP $TMP2
