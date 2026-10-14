<?php

declare(strict_types=1);

function shout(string $text, bool $loud): string
{
    if ($loud) {
        return $text . '!';
    }
    return $text;
}
