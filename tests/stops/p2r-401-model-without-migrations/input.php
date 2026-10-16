<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

// Without --migrations the tool knows no column of the model's table.
final class Journal extends Model
{
}

function code(Journal $journal): string
{
    return $journal->code;
}
