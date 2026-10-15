<?php

declare(strict_types=1);

namespace App;

use SensitiveParameter as Hide;

final readonly class Login
{
    public function __construct(#[\SensitiveParameter] public string $name)
    {
    }
}

// The alias names \SensitiveParameter, so this one is left out.
function hidden(#[Hide] string $text): int
{
    return strlen($text);
}

// Without an import this names App\SensitiveParameter.
function local(#[SensitiveParameter] string $text): int
{
    return strlen($text);
}

function counted(#[\SensitiveParameter(1)] string $text): int
{
    return strlen($text);
}
