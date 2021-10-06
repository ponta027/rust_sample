$(function(){
  var $input = $('#floatingInput');
  $input.on('change', function(event) {
    console.log($input.val());
  });
  $('#inputButton').on("click",function(event){
    console.log($input.val());
    if ( $input.val().length < 1 ){
      return;
    }
    let data={message:$input.val()};
    $.ajax({
      type:"post",
      url:"/json",
      data:JSON.stringify(data),
      contentType:"application/json",
      dataType:"json",
      success:function(json_data){
        console.log(json_data);
        update( json_data, data.message);
      }
    });
  });
  test();
})

function removeElement( obj ) {
  let index = $("tr").index(obj.parentElement.parentElement);
  // let index = obj.value;
  let url = "/json/" + (index-1) + "/del"
  $.ajax({
    type:"get",
    url:url,
    contentType:"application/json",
    dataType:"json",
    success:function(json_data){
      obj.parentElement.parentElement.remove();
    }
  });

}

function update( result ,message){
  $('#tbody').append('<tr><td>'+result.id+'</td><td>'+message+'</td><td><button type="button" class="btn btn-outline-primary" onclick="removeElement(this)" value='+result.id+'>Del</button></td></tr>');
}

function test(){
  $.ajax({
    type:"get",
    url:"/json/all",
    contentType:"application/json",
    dataType:"json",
    success:function(json_data){
      json_data.forEach(function(elem){
        console.log(elem);
        $('#tbody').append('<tr><td>'+elem.id+'</td><td>'+elem.message+'</td><td><button type="button" class="btn btn-outline-primary" onclick="removeElement(this)" value='+elem.id+'>Del</button></td></tr>');
      });
    }
  });

}

