<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

final class Tag extends Model
{
    protected $fillable = ['label'];

    public $timestamps = false;
}

function size(): int
{
    return Tag::all()->count();
}
