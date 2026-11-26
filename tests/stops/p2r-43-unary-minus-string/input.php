<?php

declare(strict_types=1);

// PHP gives -3 for "3", and throws a TypeError for a string that is not numeric.
function negated(string $s): int
{
    return -$s;
}
