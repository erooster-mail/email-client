# Icons #
subdir('icons')

metainfo_file = '@0@.metainfo.xml'.format(application_id)
desktop_file = '@0@.desktop'.format(application_id)

appstream_util = find_program('appstream-util', required: false)
desktop_file_validate = find_program('desktop-file-validate', required: false)

install_data(
    desktop_file,
    install_dir: datadir / 'applications'
)


# Validate Desktop file
if desktop_file_validate.found()
  test(
    'validate-desktop',
    desktop_file_validate,
    args: [
      datadir / 'applications' / desktop_file
    ]
  )
endif

install_data(
    metainfo_file,
    install_dir: datadir / 'metainfo'
)

# Validate Appdata
if appstream_util.found()
  test(
    'validate-appdata', appstream_util,
    args: [
      'validate', '--nonet', datadir / 'metainfo' / metainfo_file
    ]
  )
endif

#install_data(
#    '@0@.svg'.format(application_id),
#    install_dir: datadir / 'icons' / 'hicolor' / 'scalable' / 'apps'
#)

gnome.compile_resources(
  application_id,
  '@0@.gresource.xml'.format(application_id),
  gresource_bundle: true,
  source_dir: meson.current_build_dir(),
  install: true,
  install_dir: pkgdatadir
)

