from typing import Any, Callable, List, NewType, Optional, Self, Tuple, Union
from rattler.rattler import InvalidVersionError

FfiType = NewType("FfiType", Any)


class VersionBase:
    def __init__(self, inner: FfiType, from_ffi: Callable[[FfiType], Self]):
        self._inner = inner
        self.from_ffi = from_ffi

    @property
    def epoch(self) -> Optional[str]:
        """
        Gets the epoch of the version or `None` if the epoch was not defined.
        """
        return self._inner.epoch()

    def bump(self) -> Self:
        """
        Returns a new version where the last numerical segment of this version has
        been bumped.
        """
        return self.from_ffi(self._inner.bump())

    @property
    def has_local(self) -> bool:
        """
        Returns true if this version has a local segment defined.
        The local part of a version is the part behind the (optional) `+`.
        """
        return self._inner.has_local()

    def segments(self) -> List[List[Union[str, int]]]:
        """
        Returns a list of segments of the version. It does not contain
        the local segment of the version.
        """
        return self._inner.segments()

    def local_segments(self) -> List[List[Union[str, int]]]:
        """
        Returns a list of local segments of the version. It does not
        contain the non-local segment of the version.
        """
        return self._inner.local_segments()

    def as_major_minor(self) -> Optional[Tuple[int, int]]:
        """
        Returns the major and minor segments from the version.
        Requires a minimum of 2 segments in version to be split
        into major and minor, returns `None` otherwise.
        """
        return self._inner.as_major_minor()

    @property
    def is_dev(self) -> bool:
        """
        Returns true if the version contains a component name "dev",
        dev versions are sorted before non-dev version.
        """
        return self._inner.is_dev()

    def starts_with(self, other: Self) -> bool:
        """
        Checks if the version and local segment start
        same as other version.
        """

        return self._inner.starts_with(other._inner)

    def compatible_with(self, other: Self) -> bool:
        """
        Checks if this version is compatible with other version.
        Minor versions changes are compatible with older versions,
        major version changes are breaking and will not be compatible.
        """
        return self._inner.compatible_with(other._inner)

    def pop_segments(self, n: int = 1) -> Self:
        """
        Pops `n` number of segments from the version and returns
        the new version. Raises `InvalidVersionError` if version
        becomes invalid due to the operation.
        """
        new_ffi = self._inner.pop_segments(n)
        if new_ffi:
            return self.from_ffi(new_ffi)
        else:
            raise InvalidVersionError("new Version must have atleast 1 valid segment")

    def with_segments(self, start: int, stop: int) -> Self:
        """
        Returns new version with with segments ranging from `start` to `stop`.
        `stop` is exclusive. Raises `InvalidVersionError` if the provided range
        is invalid.
        """
        new_ffi = self._inner.with_segments(start, stop)
        if new_ffi:
            return self.from_ffi(new_ffi)
        else:
            raise InvalidVersionError("Invalid segment range provided")

    @property
    def segment_count(self) -> int:
        """
        Returns the number of segments in the version.
        This does not include epoch or local segment of the version
        """
        return self._inner.segment_count()

    def strip_local(self) -> Self:
        """
        Returns a new version with local segment stripped.
        """
        return self.from_ffi(self._inner.strip_local())

    def __str__(self):
        """
        Returns the string representation of the version
        """
        return self._inner.as_str()

    def __repr__(self):
        """
        Returns a representation of the version
        """
        return f'{type(self).__name__}("{self._inner.as_str()}")'

    def __hash__(self) -> int:
        """
        Computes the hash of this instance.
        """
        return self._inner.__hash__()

    def __eq__(self, other: Self) -> bool:
        """
        Returns True if this instance represents the same version as `other`.
        """
        return self._inner == other._inner

    def __ne__(self, other: Self) -> bool:
        """
        Returns True if this instance represents the same version as `other`.
        """
        return self._inner != other._inner

    def __gt__(self, other: Self) -> bool:
        """
        Returns True if this instance should be ordered *after* `other`.
        """
        return self._inner > other._inner

    def __lt__(self, other: Self) -> bool:
        """
        Returns True if this instance should be ordered *before* `other`.
        """
        return self._inner < other._inner

    def __ge__(self, other: Self) -> bool:
        """
        Returns True if this instance should be ordered *after* or at the same location
        as `other`.
        """
        return self._inner >= other._inner

    def __le__(self, other: Self) -> bool:
        """
        Returns True if this instance should be ordered *before* or at the same
        location as `other`.
        """
        return self._inner <= other._inner
