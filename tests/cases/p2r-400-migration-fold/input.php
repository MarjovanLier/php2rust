<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

// Each struct holds the columns the migrations leave: the dates are left out, note became
// not nullable, remember_token was dropped and lines came last.
final class Administration extends Model
{
}

final class Journal extends Model
{
}

final class Code extends Model
{
}

function journalLabel(Journal $journal): string
{
    return $journal->code . ': ' . $journal->note . ' (' . $journal->lines . ')';
}

function owners(Journal $journal): int
{
    return $journal->owner_id + $journal->administration_id;
}

function administrationName(Administration $administration): string
{
    return $administration->name . ' ' . $administration->uuid;
}

function codeId(Code $code): int
{
    return $code->id;
}
