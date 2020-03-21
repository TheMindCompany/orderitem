#!/bin/bash
VERSION=0.1.0

if [[ "$OSTYPE" == "linux-gnu" ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/TheMindCompany/orderitem/releases/download/${VERSION}/debian
  echo "Installing orderitem ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/orderitem
elif [[ "$OSTYPE" == "cygwin" ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/TheMindCompany/orderitem/releases/download/${VERSION}/debian
  echo "Installing orderitem ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/orderitem
elif [[ "$OSTYPE" == "debian"* ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/TheMindCompany/orderitem/releases/download/${VERSION}/debian
  echo "Installing orderitem ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/orderitem
elif [[ "$OSTYPE" == "msys" ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/TheMindCompany/orderitem/releases/download/${VERSION}/debian
  echo "Installing orderitem ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/orderitem
elif [[ "$OSTYPE" == "freebsd"* ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/TheMindCompany/orderitem/releases/download/${VERSION}/debian
  echo "Installing orderitem ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/orderitem
elif [[ "$OSTYPE" == "darwin"* ]]; then
  echo "Downloading darwin client."
  curl -LO https://github.com/TheMindCompany/orderitem/releases/download/${VERSION}/darwin
  echo "Installing orderitem ${VERSION}"
  chmod +x darwin
  mv darwin /usr/local/bin/orderitem
else
  printf "System not supported. Attempting to build from source. You must have rust installed."
  curl -LO https://github.com/TheMindCompany/orderitem/archive/${VERSION}.tar.gz
  tar -xvzf ${VERSION}.tar.gz
  cd ${VERSION}
  cargo build --release
  chmod +x target/release/orderitem
  mv target/release/orderitem /usr/local/bin/orderitem
  cd ..
  rm -rf orderitem-${VERSION}
fi

exit 0
