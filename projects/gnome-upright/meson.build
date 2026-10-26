project('gnome-upright', version: '1.0.0', meson_version: '>= 1.3')

python = import('python').find_installation('python3')
gnome = import('gnome')

subdir('data')
subdir('src')
meson.add_install_script('build-aux/meson/postinstall.py')
