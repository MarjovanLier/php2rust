<?php

declare(strict_types=1);

final readonly class Id
{
    public function __construct(public string $text) {}

    public function check(): int
    {
        return (int) $this->text[12];
    }
}

function edges(string $s): string
{
    $owned = $s . "!";
    $last = strlen($s) - 1;
    $ends = $s[0] . $s[$last];
    $first = $s[-strlen($s)];
    $more = $owned[-1] . $first;
    return $ends . $more . "abc"[1];
}

echo (new Id("8001015009087"))->check(), "\n";
echo edges("xyz"), "\n";
