<?php

declare(strict_types=1);

namespace App\Tables;

// A namespace that declares only a trait gets no module.
trait Mappings
{
    /** @var array<string, string> */
    private const array ACCENTS = ["\u{00e9}" => 'e', "\u{00e8}" => 'e', "\u{00df}" => 'ss', '  ' => ' '];
}
