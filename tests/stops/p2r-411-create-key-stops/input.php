<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

final class Code extends Model
{
    protected $fillable = ['name', 'rank'];

    public $timestamps = false;
}

// A spread element has no key of its own.
function spread(): Code
{
    return Code::create(['name' => 'a', ...['rank' => 1]]);
}

// A key read from a parameter is no string literal.
function keyed(string $column): Code
{
    return Code::create([$column => 'a', 'rank' => 1]);
}
