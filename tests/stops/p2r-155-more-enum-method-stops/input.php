<?php

declare(strict_types=1);

namespace Acme;

use function Foo\one;
use function Bar\one;
use const Other\FLAG as true;

enum Size
{
    case Small;
    case Large;

    const a = 1;
    const A = 2;
    const ?int MAYBE = 1;
    const TEXT = <<<TXT
    big
    TXT;

    public function __toString(): string
    {
        return 'size';
    }

    public function new(): int
    {
        return 1;
    }

    public function clone(): int
    {
        return 2;
    }

    public function forever(): int
    {
        return $this->forever();
    }
}

echo Size::Small->forever(), "\n";
