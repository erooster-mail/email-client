project(
    'mail-client',
    'rust',
    version: '0.1.0',    
    meson_version: '>= 0.59',
)

gnome = import('gnome')

cargo = find_program('cargo', required: true)

version = meson.project_version()
prefix = get_option('prefix')
bindir = prefix / get_option('bindir')

datadir = prefix / get_option('datadir')


dependency('glib-2.0', version: '>= 2.66')
dependency('gio-2.0', version: '>= 2.66')
dependency('gtk4', version: '>= 4.8.0')

meson.add_dist_script(
  'build-aux/dist-vendor.sh',
  meson.project_build_root() / 'meson-dist' / meson.project_name() + '-' + version,
  meson.project_source_root()
)

subdir('data')
subdir('src')

gnome.post_install(
  gtk_update_icon_cache: false,
  glib_compile_schemas: false,
  update_desktop_database: true,
)
