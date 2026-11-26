<?php

declare(strict_types=1);

// PHP rejects this file at compile time: \u{zz} is not a valid code point escape.
// Mago's parser accepts it, so this folder pins php2rust's own check.
function greeting(): string
{
    return "caf\u{zz}";
}
