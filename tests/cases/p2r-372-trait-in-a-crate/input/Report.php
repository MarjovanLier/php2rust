<?php

declare(strict_types=1);

namespace App;

use App\Enums\Kind;
use App\Processors\Splitter;

// The traits' methods are called in a module that holds neither trait.
function report(Kind $kind): string
{
    return 'Kind ' . $kind->label();
}

function counted(Splitter $splitter): int
{
    return count($splitter->lines('a,b'));
}
