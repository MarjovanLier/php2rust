<?php

declare(strict_types=1);

// PHP throws a TypeError here, because strict types do not let a float pass as an int.
function line(string $s, float $times): string
{
    return str_repeat($s, $times);
}
