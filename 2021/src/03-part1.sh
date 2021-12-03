#! /usr/bin/env bash

awk -F '' '{
  for (i = 1; i <= 12; i++) {
    ($i == 0) ? zeros[i] += 1 : ones[i] += 1;
  }
}

END {
  gamma = 0;
  epsilon = 0;
  for (i = 12; i >= 1; i--) {
    gamma += lshift(((zeros[i] > ones[i]) ? 0 : 1), 12 - i);
    epsilon += lshift(((zeros[i] > ones[i]) ? 1 : 0), 12 - i);
  }
  print gamma * epsilon
}'

