#!/usr/bin/env bash

case "${1}" in
  '5.39')
    echo ''
    ;;
  '5.40' | '5.41' | '5.42' | '5.43')
    echo 'v5-40'
    ;;
  '5.44')
    echo 'v5-44'
    ;;
  '5.45' | '5.46')
    echo 'v5-45'
    ;;
esac
