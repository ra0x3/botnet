class _BitsyError(Exception):
    pass


class InvalidPermissionError(_BitsyError):
    pass


class InvalidSettingError(_BitsyError):
    pass


class WebError(_BitsyError):
    pass


class DatabaseError(_BitsyError):
    pass


class VaultError(_BitsyError):
    pass


class ResourceDoesNotExist(_BitsyError):
    pass


class ExpiredAccessTokenError(_BitsyError):
    pass
