<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

final class Post extends Model
{
    protected $fillable = ['title', 'views'];

    public $timestamps = false;
}

// A whole program on real Eloquent: rows numbered in creation order, then read back.
$first = Post::create(['title' => 'first', 'views' => 3]);
$second = Post::create(['title' => 'second', 'views' => 5]);
echo $first->id, ' ', $second->id, "\n";
foreach (Post::all() as $post) {
    echo $post->title, ' ', $post->views, "\n";
}
