import './App.css';
import React, { useState, useEffect } from 'react';

const BASE_URL = 'http://127.0.0.1:8000'

const App = () => {
  const [posts, setPosts] = useState([]);
  const [content, setContent] = useState('');

  const [edit, setEdit] = useState(-1);
  const [editBuffer, setEditBuffer] = useState('');

  useEffect(() => {
    const fetchPost = async () => {
      const response = await fetch(BASE_URL + '/latest');
      const data = await response.json();
      setPosts(data);
    };
    fetchPost();
  }, []);

  const createPost = async (content) => {
    let response = await fetch(BASE_URL + '/newpost', {
      method: 'POST',
      body: JSON.stringify({
        content: content
      }),
      headers: {
        'Content-type': 'text/plain; charset=UTF-8',
      },
    });

    let data = await response.json();
    setPosts((post) => [data, ...posts]);
    setContent('');
  };

  const editPost = async (id, content) => {
    let response = await fetch(BASE_URL + '/editpost', {
      method: 'PUT',
      body: JSON.stringify({
        id: id,
        content: content
      }),
      headers: {
        'Content-type': 'text/plain; charset=UTF-8',
      },
    });

    let data = await response.json();
    setPosts(posts.map((post) => {
      if (post.id === id) {
        post.content = content;
      }
      return post;
    }));
    setEditBuffer('');
  };

  const deletePost = async (id) => {
    let response = await fetch(BASE_URL + '/deletepost', {
      method: 'DELETE',
      body: JSON.stringify({
        id: id
      }),
      headers: {
        'Content-type': 'text/plain; charset=UTF-8',
      },
    });

    if (response.status === 200) {
      let data = await response.json();
      setPosts(posts.filter((post) => post.id !== id));
    } else {
      return;
    }
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    createPost(content);
  };

  const handleEditState = (e, id) => {
    e.preventDefault();
    setEdit(id);
  }

  const handleEdit = (e, id) => {
    e.preventDefault();
    editPost(id, editBuffer);
    setEdit(-1);
  }

  const handleDelete = (e, id) => {
    e.preventDefault();
    deletePost(id);
  };

  const postBuilder = (post) => {
    if (post.id === edit) {
      return(
        <div className='post-container' key={post.id}>
          <form onSubmit={(e) => handleEdit(e, post.id)}>
            <input type='text' className='form-control' value={editBuffer} onChange={(e) => setEditBuffer(e.target.value)} />
            <button type="submit">Submit</button>
            <button onClick={(e) => handleEditState(e, -1)}>Cancel</button>
          </form>
          <p className='post-likes'>{post.likes}</p>
          <p className='post-date'>{post.post_date}</p>
          <button onClick={(e) => handleDelete(e, post.id)}>Delete</button>
          
        </div>
      );
    } else {
      return (
        <div className='post-container' key={post.id}>
          <p className='post-content'>{post.content}</p>
          <p className='post-likes'>{post.likes}</p>
          <p className='post-date'>{post.post_date}</p>
          <button onClick={(e) => handleDelete(e, post.id)}>Delete</button>
          <button onClick={(e) => handleEditState(e, post.id)}>Edit</button>
        </div>
      );
    }
  }

  return (
    <div className="App">
      <form onSubmit={handleSubmit}>
        <input type="text" className="form-control" value={content} onChange={(e) => setContent(e.target.value)} />
        <button type="submit">Send</button>
      </form>
      {posts.map(postBuilder)}
    </div>
  );

};

export default App;
