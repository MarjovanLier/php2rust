<?php

declare(strict_types=1);

namespace Acme;

use Foo\Bar;
use Baz\Bar;
use Foo\Box;
use function Foo\helper;
use Foo\Thing as int;
use const Foo\LIMIT;
use const Foo\limit;

final readonly class Box
{
    public function __construct(public int $n) {}
}

function helper(): int
{
    return 1;
}

echo helper(), "\n";
