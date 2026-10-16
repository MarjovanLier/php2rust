<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

final class Tag extends Model
{
    protected $fillable = ['label'];

    public $timestamps = false;
}

// The included Model declares findOrFail, so no patch gives it create, and Mago types the
// result as mixed (P2R-410, P2R-421).
function made(): Tag
{
    return Tag::create(['label' => 'a']);
}
