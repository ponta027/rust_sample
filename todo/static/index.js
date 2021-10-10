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
        insertData( json_data, data.message);
      }
    });
  });
  getAllElement();
})

function removeElement( obj ) {
  let index = obj.value;
  let url = "/json/" + index + "/del"
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

function changeEvent(elem){
  const val = this.args.value;
  const url = "/json/"+ this.args.parentElement.previousSibling.textContent;
  let data={message:val};
  $.ajax({
    type:"post",
    url:url,
    data:JSON.stringify(data),
    contentType:"application/json",
    dataType:"json",
    success:function(json_data){
      console.log(json_data);
    }
  });
}


function insertData( result ,message){
  insertElement($('#tbody'),result.id , message);
}

function insertElement( elem , idx, id , message ){
  let line_data = '<tr><td>'+id+'</td><td><input type="email" class="form-control" id="editValue" value="'+message+'"/></td><td><button type="button" class="btn btn-outline-primary" onclick="removeElement(this)" value='+id+'>Del</button></td></tr>';

  elem.append(line_data);
  let form_elem = elem.find(".form-control")[idx];
  form_elem.addEventListener("change", {args:form_elem,handleEvent:changeEvent,id:id});

}
function getAllElement(){
  $.ajax({
    type:"get",
    url:"/json/all",
    contentType:"application/json",
    dataType:"json",
    success:function(json_data){
      let idx = 0;
      json_data.forEach(function(elem){
        console.log(elem);
        insertElement($('#tbody'),idx , elem.id , elem.message);
        idx++;
      });
    }
  });

}

