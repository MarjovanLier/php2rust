<?php

declare(strict_types=1);

final readonly class Code
{
    public function __construct(public string $text) {}

    public function numeric(): bool
    {
        return ctype_digit($this->text);
    }
}

function check(string $s): string
{
    $owned = $s . "1";
    if (ctype_digit($s) && ctype_digit($owned) && \ctype_digit("12")) {
        return "digits";
    }
    return "other";
}

echo check("42"), " ", check("4x"), " ", check(""), "\n";
if ((new Code("0412"))->numeric() && !(new Code("04 12"))->numeric()) {
    echo "code\n";
}
