<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

final class Tag extends Model
{
    protected $fillable = ['label'];

    public $timestamps = false;
}

// Only all() reads the store, so its impl block holds store_all and no store_get (P2R-441).
function labels(): string
{
    $out = '';
    foreach (Tag::all() as $tag) {
        $out .= $tag->label;
    }
    return $out . count(Tag::all());
}

echo labels(), "\n";
Tag::create(['label' => 'a']);
Tag::create(['label' => 'b']);
echo labels(), "\n";
