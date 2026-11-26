<?php

declare(strict_types=1);

// PHP throws a TypeError here, because strict types do not let an int pass as a string.
function masked(int $n): string
{
    $out = preg_replace('/\d/', '#', $n) ?? '';
    return is_string($out) ? $out : '';
}
