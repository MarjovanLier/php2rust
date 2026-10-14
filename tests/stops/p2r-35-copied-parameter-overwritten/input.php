<?php

declare(strict_types=1);

function excited(string $text): string
{
    $text .= '!';
    return $text;
}

function replaced(string $text): string
{
    $text = 'new';
    return $text;
}

echo excited('hi'), replaced('old'), "\n";
