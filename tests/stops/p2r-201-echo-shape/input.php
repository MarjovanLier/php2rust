<?php

declare(strict_types=1);

// PHP prints "Array" here, with an "Array to string conversion" warning.
/**
 * @param array{name: string, age: int} $person
 */
function show(array $person): void
{
    echo $person, "\n";
}
