#! /bin/sh
git clone https://github.com/yvan-sraka/YeAST /tmp/yeast-sources
pushd /tmp/yeast-sources
./configure
make install
popd
rm -rf /tmp/yeast-sources
