<?php

declare(strict_types=1);

namespace Acme\Text;

use function Acme\Text\shout as strlen_of;

final readonly class Exception
{
    public function __construct(public int $code) {}
}

function shout(string $text): int
{
    return \strlen($text) + strlen($text) + intdiv(\count([1, 2]), 2);
}

function twice(string $text): int
{
    return namespace\shout($text) + \Acme\Text\shout($text);
}

/**
 * @param list<int> $numbers
 */
function total(array $numbers, ?int $extra): int
{
    return \count($numbers) + ($extra ?? 0);
}

$none = [];
$nothing = null;
echo total($none, $nothing), "\n";

echo shout('ab'), "\n";
echo twice('abc'), "\n";
echo strlen_of('abcd'), "\n";
echo (new Exception(7))->code, "\n";
