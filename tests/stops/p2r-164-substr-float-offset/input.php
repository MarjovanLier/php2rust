<?php

declare(strict_types=1);

// PHP throws a TypeError here, because strict types do not let a float pass as an int.
function tail(string $s, float $from): string
{
    return substr($s, $from);
}
