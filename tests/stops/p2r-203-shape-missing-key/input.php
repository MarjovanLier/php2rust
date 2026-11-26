<?php

declare(strict_types=1);

// PHP warns of an undefined key here, then throws a TypeError on the null it returns.
/**
 * @param array{name: string, age: int} $person
 */
function city(array $person): string
{
    return $person['city'];
}
