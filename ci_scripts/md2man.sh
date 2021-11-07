#!/usr/bin/bash

rm man/cifra.1.gz
cp README.md man/.
sed -i '/^\[\!\[/d' man/README.md
pandoc --standalone --to man man/README.md -o man/cifra.1
rm man/README.md
gzip man/cifra.1
echo "Manpage built"
