tmpdir=`mktemp -d`
echo $tmpdir
comp=$1
dest=$2
c=`basename $comp .tar.xz`
echo $c
tar xvyf $comp -C $tmpdir
bash $tmpdir/$c/install.sh --prefix=$dest
rm -rf $tmpdir/$c
rm -rf $tmpdir
