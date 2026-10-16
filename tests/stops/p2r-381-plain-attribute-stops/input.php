<?php

declare(strict_types=1);

namespace App;

use Spatie\LaravelData\Attributes\MapInputName;

// laravel-data's attributes are left out only on a class and on the promoted parameters of
// its constructor (P2R-381).
final class Places
{
    #[MapInputName('a')]
    public string $declared = '';

    public function __construct(
        #[MapInputName('b')]
        public string $promoted,
    ) {}

    public function method(#[MapInputName('c')] string $parameter): string
    {
        return $parameter;
    }

    #[MapInputName('d')]
    public function marked(): string
    {
        return $this->promoted;
    }
}

function plain(#[MapInputName('e')] string $parameter): string
{
    return $parameter;
}

#[MapInputName('f')]
enum Kind
{
    case One;
}

#[MapInputName('g')]
interface Named
{
}
