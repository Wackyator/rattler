from __future__ import annotations

from rattler.rattler import PyVersion
from rattler.version import VersionBase


class Version(VersionBase):
    """
    This class implements an order relation between version strings.
    Version strings can contain the usual alphanumeric characters
    (A-Za-z0-9), separated into segments by dots and underscores.
    Empty segments (i.e. two consecutive dots, a leading/trailing
    underscore) are not permitted. An optional epoch number - an
    integer followed by '!' - can precede the actual version string
    (this is useful to indicate a change in the versioning scheme itself).
    Version comparison is case-insensitive.
    """

    def __init__(self, version: str):
        if isinstance(version, str):
            self._version = PyVersion(version)
            super().__init__(self._version, Version._from_py_version)
        else:
            raise TypeError(
                "Version constructor received unsupported type "
                f" {type(version).__name__!r} for the `version` parameter"
            )

    @classmethod
    def _from_py_version(cls, py_version: PyVersion) -> Version:
        """Construct Rattler version from FFI PyVersion object."""
        version = cls.__new__(cls)
        version._version = py_version
        return version
