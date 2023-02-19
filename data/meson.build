application_id = 'dev.nordgedanken.Email'
metainfo_file = '@0@.metainfo.xml'.format(application_id)
desktop_file = '@0@.desktop'.format(application_id)

appstream_util = find_program('appstream-util', required: false)
desktop_file_validate = find_program('desktop-file-validate', required: false)

# Validate Desktop file
if desktop_file_validate.found()
  test(
    'validate-desktop',
    desktop_file_validate,
    args: [
      desktop_file
    ]
  )
endif


install_data(
    desktop_file,
    install_dir: datadir / 'applications'
)

# Validate Appdata
if appstream_util.found()
  test(
    'validate-appdata', appstream_util,
    args: [
      'validate', '--nonet', metainfo_file
    ]
  )
endif

install_data(
    metainfo_file,
    install_dir: datadir / 'metainfo'
)

#install_data(
#    '@0@.svg'.format(application_id),
#    install_dir: datadir / 'icons' / 'hicolor' / 'scalable' / 'apps'
#)
