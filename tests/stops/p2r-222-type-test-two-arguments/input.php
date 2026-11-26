<?php

declare(strict_types=1);

// PHP throws an ArgumentCountError here: is_scalar() takes one argument.
function scalar(int $n, string $s): bool
{
    return is_scalar($n, $s);
}
