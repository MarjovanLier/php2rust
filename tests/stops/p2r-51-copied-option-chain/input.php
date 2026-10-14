<?php

declare(strict_types=1);

function pick(bool $some, string $text): string
{
    $second = null;
    if ($some) {
        $second = "set";
    }
    $a = $second ?? substr($text, 1, 2);
    $b = $second ?? "x";
    return $a . $b;
}

echo pick(true, "hello"), " ", pick(false, "hello"), "\n";
